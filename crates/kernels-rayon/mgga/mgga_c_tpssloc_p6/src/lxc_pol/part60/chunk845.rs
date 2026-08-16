//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 845/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk845(t5979: f64, t7286: f64, t7285: f64, t5975: f64, t27820: f64, t8002: f64, t1238: f64, t24589: f64, t27808: f64, t27818: f64, t29795: f64, t29798: f64, t29804: f64, t29809: f64, t5055: f64, t6268: f64, t7283: f64, t7351: f64, t8088: f64) -> f64 {
    let t29812 = t7286 * t5979;
    let t29813 = t7285 * t29812;
    let t29816 = t7286 * t5975;
    let t29817 = t7285 * t29816;
    let t29822 = t27820 * t8002;
    let t29825 = -0.14621636149762012769e-1_f64 * t27808 - t1238 * t29795 - 6.0_f64 * t1238 * t29798 - t7351 * t6268 + 0.54831135561607547884e-2_f64 * t27818 + 0.16449340668482264365e-1_f64 * t7283 * t29804 + 0.54831135561607547884e-2_f64 * t24589 * t29809 - 0.27415567780803773942e-2_f64 * t7283 * t29813 - 0.54831135561607547884e-2_f64 * t7283 * t29817 - 2.0_f64 * t5055 * t8088 + 0.54831135561607547884e-2_f64 * t24589 * t29822;
    t29825
}
