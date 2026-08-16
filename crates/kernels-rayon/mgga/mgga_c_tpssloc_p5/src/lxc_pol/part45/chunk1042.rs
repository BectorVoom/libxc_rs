//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1042/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1042(t113836: f64, t1862: f64, t8513: f64, t39054: f64, t8511: f64, t39063: f64, t2241: f64, t8514: f64, t31687: f64, t9239: f64, t31677: f64, t2244: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t115863 = t8513 * t113836 * t1862;
    let t115866 = t39054 * t8511;
    let t115871 = t39063 * t8511;
    let t115873 = t8513 * t8514 * t2241;
    let t115876 = t9239 * t31687;
    let t115877 = t115876 * t31677;
    let t115880 = t8513 * t8514 * t2244;
    (t115863, t115866, t115871, t115873, t115877, t115880)
}
