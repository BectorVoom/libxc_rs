//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1321/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1321<F: Float>(t13314: F, t303: F, t356: F, t26717: F, t27895: F, t46978: F, t8033: F, t2173: F, t26753: F, t2842: F, t4556: F, t1009: F, t26697: F, t26732: F, t26767: F, t27832: F, t291: F, t330: F, t5314: F, t93638: F, t93653: F, t93662: F, t93664: F) -> (F, F, F, F) {
    let t96298 = t303 * t356 * t13314;
    let t96302 = F::new(0.61836467013888888889e-4) * t27895 * t26717;
    let t96305 = t46978 * t8033;
    let t96306 = t2173 * t96305;
    let t96311 = t2842 * t26753 * t4556;
    let t96313 = -F::new(0.23168402777777777778e-3) * t27832 * t26767 - F::new(0.30891203703703703704e-3) * t27832 * t26697 - F::new(0.46336805555555555556e-3) * t2173 * t5314 * t291 * t1009 * t330 - F::new(0.61890573922526041668e-5) * t93638 + F::new(0.1621345679012345679e-1) * t96298 + F::new(0.46336805555555555556e-3) * t93653 + t96302 - F::new(0.41224311342592592593e-4) * t93662 - F::new(0.92754700520833333335e-4) * t93664 - F::new(0.15445601851851851852e-3) * t96306 + F::new(0.92754700520833333333e-4) * t27895 * t26732 + F::new(0.55273148148148148146e-2) * t96311;
    (t96298, t96305, t96311, t96313)
}
