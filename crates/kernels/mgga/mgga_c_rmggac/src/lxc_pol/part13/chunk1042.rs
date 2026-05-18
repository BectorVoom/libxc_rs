//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1042/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1042<F: Float>(t38775: F, t38818: F, t1356: F, t27075: F, t37218: F, t38752: F, t38755: F, t38757: F, t38760: F, t38764: F, t38780: F, t38784: F, t38793: F, t38796: F, t38799: F, t38802: F, t38807: F, t38813: F, t8041: F) -> F {
    let t42740 = F::new(0.36366215538993788974e-1) * t38775;
    let t42749 = F::new(0.1440846329149835838e-2) * t38818;
    let t42750 = -F::new(0.72042316457491791901e-3) * t38752 - F::new(0.72042316457491791901e-3) * t38755 - F::new(0.30487649791575028312e-3) * t38757 - F::new(0.72042316457491791901e-3) * t38760 - F::new(0.72042316457491791901e-3) * t38764 - F::new(0.11974241701863808564e0) * t1356 * t8041 * t27075 + t42740 + F::new(0.85129199786595678799e-5) * t38780 + F::new(0.20001418546446583936e0) * t38784 + F::new(0.71845450211182851384e0) * t38793 - F::new(0.35922725105591425692e0) * t38796 - F::new(0.14369090042236570277e1) * t38799 - F::new(0.35922725105591425692e0) * t38802 - t37218 + F::new(0.40911992481368012596e-1) * t38807 + F::new(0.2993560425465952141e-1) * t38813 + t42749;
    t42750
}
