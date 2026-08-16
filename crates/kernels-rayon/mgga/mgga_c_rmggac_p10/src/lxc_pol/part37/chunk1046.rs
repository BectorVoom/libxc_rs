//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1046/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1046(t74015: f64, t74041: f64, t74049: f64, t76821: f64, t76823: f64, t76828: f64, t76829: f64, t76830: f64, t76831: f64, t76832: f64, t76834: f64, t76835: f64, t76836: f64, t76837: f64, t76838: f64, t76841: f64) -> f64 {
    let t80028 = t76821 - 0.72714524817717142305e-5_f64 * t74015 - t76823 + t76828 - t76829 - t76830 + t76831 + t76832 - 0.72714524817717142305e-5_f64 * t74041 + t76834 - t76835 + t74049 - t76836 + t76837 + t76838 - t76841;
    t80028
}
