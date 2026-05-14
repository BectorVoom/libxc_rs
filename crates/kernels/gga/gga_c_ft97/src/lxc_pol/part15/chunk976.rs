//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 976/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk976<F: Float>(t5053: F, t4934: F, t21337: F, t21382: F, t21309: F, t4952: F, t6: F, t30852: F, t65693: F, t21333: F, t39: F, t4960: F, t1111: F, t13474: F, t13475: F, t17825: F, t17833: F, t21224: F, t21225: F, t21233: F, t21262: F, t21319: F, t21330: F, t21331: F, t21332: F, t21374: F, t21386: F, t30853: F, t4948: F, t4949: F, t4950: F, t4951: F, t4953: F, t4978: F, t65695: F, t65702: F, t65743: F, t66092: F, t79253: F, t79402: F, t79622: F, t79950: F, t79972: F) -> (F, F, F, F) {
    let t88289 = t5053 * t5053;
    let t88294 = t4934 * t4934;
    let t88310 = t21337 * t21382;
    let t88314 = t21309 * t6 * t4952;
    let t88320 = t30852 * t65693;
    let t88337 = t4960 * t39 * t21333;
    let t88352 = -0.279058811357253504e-1 * t13474 * t13475 * t21319 - 0.23709522591370051951e-1 * t21224 * t1111 + 0.1422571355482203117e0 * t21225 * t1111 - 0.12803864807119409228e-1 * t4949 * t4950 * t4951 * t21233 - 0.52379446938215765024e-3 * t17825 * t88310 - 0.66163511921956755822e-4 * t66092 * t88314 - 0.16540877980489188956e-2 * t79402 * t4948 * t4953 + 0.38465647900339007384e-4 * t88320 * t65695 - 0.9804408003987596673e-5 * t21330 * t4978 * t39 * t21333 + 0.26189723469107882512e-2 * t4949 * t21386 * t21382 + 0.12418916805050955786e-3 * t21330 * t21331 * t21332 * t21262 - 0.1744777815077289385e-3 * t17833 * t88310 + 0.16329414088222212441e-6 * t79950 * t88337 - 0.27568129967481981592e-3 * t4949 * t21374 * t6 * t4952 - 0.30589033253692324537e-6 * t65743 * t88314 + 0.19232823950169503692e-4 * t79253 * t79622 - 0.19232823950169503692e-4 * t30853 * t65702 + 0.19608816007975193346e-5 * t79972 * t88337;
    (t88289, t88294, t88314, t88352)
}
