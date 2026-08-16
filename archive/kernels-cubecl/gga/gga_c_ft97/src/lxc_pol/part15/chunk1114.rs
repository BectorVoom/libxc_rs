//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1114/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1114<F: Float>(t1111: F, t13474: F, t13475: F, t17825: F, t17833: F, t21224: F, t21225: F, t21233: F, t21262: F, t21319: F, t21330: F, t21331: F, t21332: F, t21333: F, t21374: F, t21382: F, t21386: F, t30853: F, t39: F, t4948: F, t4949: F, t4950: F, t4951: F, t4952: F, t4953: F, t4978: F, t6: F, t65695: F, t65702: F, t65743: F, t66092: F, t79253: F, t79402: F, t79622: F, t79950: F, t79972: F, t88310: F, t88314: F, t88320: F, t88337: F) -> F {
    let t88352 = -F::cast_from(0.279058811357253504e-1_f64) * t13474 * t13475 * t21319 - F::cast_from(0.23709522591370051951e-1_f64) * t21224 * t1111 + F::cast_from(0.1422571355482203117e0_f64) * t21225 * t1111 - F::cast_from(0.12803864807119409228e-1_f64) * t4949 * t4950 * t4951 * t21233 - F::cast_from(0.52379446938215765024e-3_f64) * t17825 * t88310 - F::cast_from(0.66163511921956755822e-4_f64) * t66092 * t88314 - F::cast_from(0.16540877980489188956e-2_f64) * t79402 * t4948 * t4953 + F::cast_from(0.38465647900339007384e-4_f64) * t88320 * t65695 - F::cast_from(0.9804408003987596673e-5_f64) * t21330 * t4978 * t39 * t21333 + F::cast_from(0.26189723469107882512e-2_f64) * t4949 * t21386 * t21382 + F::cast_from(0.12418916805050955786e-3_f64) * t21330 * t21331 * t21332 * t21262 - F::cast_from(0.1744777815077289385e-3_f64) * t17833 * t88310 + F::cast_from(0.16329414088222212441e-6_f64) * t79950 * t88337 - F::cast_from(0.27568129967481981592e-3_f64) * t4949 * t21374 * t6 * t4952 - F::cast_from(0.30589033253692324537e-6_f64) * t65743 * t88314 + F::cast_from(0.19232823950169503692e-4_f64) * t79253 * t79622 - F::cast_from(0.19232823950169503692e-4_f64) * t30853 * t65702 + F::cast_from(0.19608816007975193346e-5_f64) * t79972 * t88337;
    t88352
}
