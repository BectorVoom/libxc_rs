//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1145/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1145(t3602: f64, t39922: f64, t6087: f64, t37755: f64, t6064: f64, t3606: f64, t2195: f64, t37754: f64, t37769: f64, t7606: f64, t11842: f64, t1584: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39924 = t39922 * t3602 * t6087;
    let t39927 = t37755 * t3602 * t6064;
    let t39930 = t37755 * t3606 * t6087;
    let t39935 = t2195 * t37754;
    let t39937 = t39935 * t3606 * t6064;
    let t39939 = t37769 * t7606;
    let t39941 = t1584 * t11842;
    (t39924, t39927, t39930, t39937, t39939, t39941)
}
