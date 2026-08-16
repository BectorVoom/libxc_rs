//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1277/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1277(t1332: f64, t1336: f64, t22710: f64, t22871: f64, t22873: f64, t22874: f64, t22879: f64, t3777: f64, t3793: f64, t3856: f64, t81066: f64, t81069: f64, t81073: f64, t81075: f64, t81076: f64, t81080: f64, t81083: f64, t81087: f64, t81092: f64, t81097: f64, t81099: f64, t81105: f64, t81115: f64) -> f64 {
    let t81117 = 0.24674011002723396547e-1_f64 * t81066 - 0.12337005501361698274e-1_f64 * t81069 - t81073 - t81075 + 0.78134368175290755733e-1_f64 * t81076 + 3.0_f64 * t1332 * t22871 - 0.15626873635058151147e0_f64 * t81080 + 0.49348022005446793095e-1_f64 * t81083 - 0.19739208802178717238e0_f64 * t81087 - 0.24674011002723396548e-1_f64 * t81092 - 0.24674011002723396548e-1_f64 * t81097 + 0.57572692339687925277e-1_f64 * t81099 - 3.0_f64 * t3777 * t22879 - 6.0_f64 * t3777 * t22874 + 6.0_f64 * t1336 * t81105 * t3793 + 6.0_f64 * t3777 * t22710 - 3.0_f64 * t1336 * t22873 * t3856 + 0.12337005501361698274e-1_f64 * t81115;
    t81117
}
