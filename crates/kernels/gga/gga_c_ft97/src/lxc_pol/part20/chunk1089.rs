//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1089/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1089<F: Float>(t13631: F, t27510: F, t625: F, t6818: F, t6820: F, t6815: F, t1120: F, t6054: F, t52608: F, t689: F, t52369: F, t24294: F, t27506: F, t108456: F, t108460: F, t108464: F, t108468: F, t13520: F, t13522: F, t24306: F, t24311: F, t27566: F, t27584: F, t27601: F, t3774: F, t6023: F, t6055: F, t65685: F, t65747: F, t65754: F, t96448: F, t96630: F) -> (F, F, F, F, F, F) {
    let t108635 = t27510 * t13631;
    let t108639 = t6818 * t625 * t6820;
    let t108640 = t6815 * t108639;
    let t108647 = t6054 * t1120;
    let t108650 = t52608 * t689;
    let t108660 = t52369 * t689;
    let t108673 = t27506 * t24294;
    let t108676 = -0.60102574844279699039e-6 * t65685 * t27601 + 0.12768721675925925926e-1 * t6055 * t108635 + 0.10091343167942740398e-3 * t108640 - 0.38306165027777777778e-1 * t96448 - 0.13784064983740990796e-3 * t27566 * t65747 + 0.91830411319857336049e-5 * t27566 * t65754 + 0.13810404665630505674e-4 * t24306 * t108647 - 0.10338048737805743098e-4 * t3774 * t6023 * t108650 - 0.51690243689028715488e-5 * t3774 * t6023 * t108456 + 0.10330921273483950306e-5 * t3774 * t24311 * t108460 - 0.3443640424494650102e-5 * t3774 * t24311 * t108660 - 0.1721820212247325051e-5 * t3774 * t24311 * t108464 + 0.28677218675336554254e-7 * t3774 * t96630 * t108468 + 0.27568129967481981592e-3 * t13520 * t27584 * t13522 + 0.17024962234567901235e-1 * t6055 * t108673;
    (t108635, t108639, t108650, t108660, t108673, t108676)
}
