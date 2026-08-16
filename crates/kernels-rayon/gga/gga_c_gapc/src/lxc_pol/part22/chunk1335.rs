//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1335/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1335(t12295: f64, t699: f64, t10794: f64, t24007: f64, t13296: f64, t2470: f64, t3832: f64, t35720: f64, t35722: f64, t35725: f64, t35727: f64, t35732: f64, t35736: f64, t35738: f64, t35741: f64, t35745: f64, t35747: f64, t35749: f64, t35753: f64, t35755: f64) -> (f64, f64, f64, f64) {
    let t36124 = t699 * t12295;
    let t36127 = 6.0_f64 * t24007 * t10794;
    let t36130 = 24.0_f64 * t13296 * t3832 * t2470;
    let t36144 = 0.29357452990051769742e-5_f64 * t35720 - 0.46971924784082831588e-4_f64 * t35722 - 0.14678726495025884871e-5_f64 * t35725 + 0.34197428278281706076e-6_f64 * t35727 + 0.19948499828997661878e-6_f64 * t35732 + 0.68394856556563412152e-6_f64 * t35736 + 0.34197428278281706076e-6_f64 * t35738 - 0.9785817663350589914e-7_f64 * t35741 + 0.12843885683147649262e-5_f64 * t35745 - 0.27053965482373971918e-4_f64 * t35747 + 0.61555370900907070936e-5_f64 * t35749 + 0.83516082266099274564e-5_f64 * t35753 + 0.18951074837547778784e-5_f64 * t35755;
    (t36124, t36127, t36130, t36144)
}
