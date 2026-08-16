//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1205/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1205(t15059: f64, t986: f64, t3270: f64, t3269: f64, t10610: f64, t3465: f64, t42454: f64, t42392: f64, t1115: f64, t2892: f64, t36986: f64, t3275: f64, t3472: f64, t42851: f64) -> (f64, f64, f64, f64, f64) {
    let t44011 = t15059 * t986;
    let t44012 = t3270 * t44011;
    let t44014 = t3269 * t44012 / 2.0_f64;
    let t44017 = 3.0_f64 / 2.0_f64 * t10610 * t3465 * t42454;
    let t44020 = 3.0_f64 * t10610 * t3465 * t42392;
    let t44022 = t3270 * t1115 * t2892;
    let t44024 = 3.0_f64 / 2.0_f64 * t36986 * t44022;
    let t44027 = 5.0_f64 / 8.0_f64 * t3275 * t3472 * t42851;
    (t44014, t44017, t44020, t44024, t44027)
}
