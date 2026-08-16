//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 828/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk828(t5728: f64, t5955: f64, t5727: f64, t758: f64, t2923: f64, t5703: f64, t302: f64, t2030: f64, t655: f64, t2124: f64, t2105: f64, t2106: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5956 = t5728 * t5955;
    let t5957 = t5727 * t5956;
    let t5958 = t758 * t5957;
    let t5961 = t5703 * t2923;
    let t5962 = t302 * t5961;
    let t5965 = t2030 * t655;
    let t5966 = t2124 * t5965;
    let t5967 = t2105 * t5966;
    let t5970 = t2124 * t2106;
    (t5956, t5957, t5958, t5961, t5962, t5965, t5966, t5967, t5970)
}
