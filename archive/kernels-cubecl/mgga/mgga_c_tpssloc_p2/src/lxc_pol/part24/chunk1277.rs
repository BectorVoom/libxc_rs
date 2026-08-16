//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1277/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1277<F: Float>(t1332: F, t1336: F, t22710: F, t22871: F, t22873: F, t22874: F, t22879: F, t3777: F, t3793: F, t3856: F, t81066: F, t81069: F, t81073: F, t81075: F, t81076: F, t81080: F, t81083: F, t81087: F, t81092: F, t81097: F, t81099: F, t81105: F, t81115: F) -> F {
    let t81117 = F::cast_from(0.24674011002723396547e-1_f64) * t81066 - F::cast_from(0.12337005501361698274e-1_f64) * t81069 - t81073 - t81075 + F::cast_from(0.78134368175290755733e-1_f64) * t81076 + F::cast_from(3.0_f64) * t1332 * t22871 - F::cast_from(0.15626873635058151147e0_f64) * t81080 + F::cast_from(0.49348022005446793095e-1_f64) * t81083 - F::cast_from(0.19739208802178717238e0_f64) * t81087 - F::cast_from(0.24674011002723396548e-1_f64) * t81092 - F::cast_from(0.24674011002723396548e-1_f64) * t81097 + F::cast_from(0.57572692339687925277e-1_f64) * t81099 - F::cast_from(3.0_f64) * t3777 * t22879 - F::cast_from(6.0_f64) * t3777 * t22874 + F::cast_from(6.0_f64) * t1336 * t81105 * t3793 + F::cast_from(6.0_f64) * t3777 * t22710 - F::cast_from(3.0_f64) * t1336 * t22873 * t3856 + F::cast_from(0.12337005501361698274e-1_f64) * t81115;
    t81117
}
