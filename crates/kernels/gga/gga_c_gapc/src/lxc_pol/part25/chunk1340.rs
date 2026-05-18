//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1340/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1340<F: Float>(t1112: F, t1616: F, t9370: F, t10794: F, t24007: F, t13296: F, t2470: F, t3832: F, t35720: F, t35722: F, t35725: F, t35727: F, t35732: F, t35736: F, t35738: F, t35741: F, t35745: F, t35747: F, t35749: F, t35753: F, t35755: F) -> (F, F, F, F) {
    let t36119 = F::new(2.0) * t1616 * t1112 * t9370;
    let t36127 = F::new(6.0) * t24007 * t10794;
    let t36130 = F::new(24.0) * t13296 * t3832 * t2470;
    let t36144 = F::new(0.29357452990051769742e-5) * t35720 - F::new(0.46971924784082831588e-4) * t35722 - F::new(0.14678726495025884871e-5) * t35725 + F::new(0.34197428278281706076e-6) * t35727 + F::new(0.19948499828997661878e-6) * t35732 + F::new(0.68394856556563412152e-6) * t35736 + F::new(0.34197428278281706076e-6) * t35738 - F::new(0.9785817663350589914e-7) * t35741 + F::new(0.12843885683147649262e-5) * t35745 - F::new(0.27053965482373971918e-4) * t35747 + F::new(0.61555370900907070936e-5) * t35749 + F::new(0.83516082266099274564e-5) * t35753 + F::new(0.18951074837547778784e-5) * t35755;
    (t36119, t36127, t36130, t36144)
}
