//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 570/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk570(t1248: f64, t4065: f64, t7736: f64, t1249: f64, t7740: f64, t7744: f64, t4049: f64, t4061: f64, t6020: f64, t6066: f64, t7914: f64, t7917: f64, t7920: f64, t7932: f64, t7939: f64, t7945: f64, t7947: f64) -> (f64, f64, f64, f64) {
    let t7951 = t1248 * t4065 * t7736;
    let t7954 = t1248 * t1249 * t7740;
    let t7957 = t1248 * t1249 * t7744;
    let t7959 = -0.9494625e0_f64 * t7932 + 0.1898925e1_f64 * t7939 + t4049 + 0.19931111111111111111e0_f64 * t6020 - 0.19931111111111111111e0_f64 * t7914 + 0.59793333333333333334e0_f64 * t7917 - 0.29896666666666666667e0_f64 * t7920 + 0.15358125e0_f64 * t7945 + 0.3071625e0_f64 * t7947 + t4061 + 0.21908444444444444444e0_f64 * t6066 - 0.5477111111111111111e-1_f64 * t7951 + 0.32862666666666666666e0_f64 * t7954 - 0.16431333333333333333e0_f64 * t7957;
    (t7951, t7954, t7957, t7959)
}
