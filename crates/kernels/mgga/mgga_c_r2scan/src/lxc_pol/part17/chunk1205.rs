//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1205/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1205<F: Float>(t15059: F, t986: F, t3270: F, t3269: F, t10610: F, t3465: F, t42454: F, t42392: F, t1115: F, t2892: F, t36986: F, t3275: F, t3472: F, t42851: F) -> (F, F, F, F, F) {
    let t44011 = t15059 * t986;
    let t44012 = t3270 * t44011;
    let t44014 = t3269 * t44012 / F::cast_from(2.0_f64);
    let t44017 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t10610 * t3465 * t42454;
    let t44020 = F::cast_from(3.0_f64) * t10610 * t3465 * t42392;
    let t44022 = t3270 * t1115 * t2892;
    let t44024 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t36986 * t44022;
    let t44027 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t3275 * t3472 * t42851;
    (t44014, t44017, t44020, t44024, t44027)
}
