//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1397/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1397<F: Float>(t34804: F, t34808: F, t34811: F, t34813: F, t34822: F, t34824: F, t34826: F, t34830: F, t34832: F, t34834: F, t34839: F, t34846: F, t34849: F, t34851: F, t34853: F, t34856: F, t34858: F, t34860: F, t34866: F, t34868: F, t34870: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37080 = F::cast_from(0.9275345110817126956e-4_f64) * t34804;
    let t37082 = F::cast_from(0.77294542590142724634e-6_f64) * t34808;
    let t37083 = F::cast_from(0.1374296967252737644e-5_f64) * t34811;
    let t37084 = F::cast_from(0.27434213293897156973e-6_f64) * t34813;
    let t37086 = F::cast_from(0.36016197965321395821e-6_f64) * t34822;
    let t37087 = F::cast_from(0.8446756622939173539e-6_f64) * t34824;
    let t37088 = F::cast_from(0.50603841145833333336e-5_f64) * t34826;
    let t37089 = F::cast_from(0.43440462632258606772e-4_f64) * t34830;
    let t37090 = F::cast_from(0.80045999977926802214e-7_f64) * t34832;
    let t37091 = F::cast_from(0.64036799982341441771e-6_f64) * t34834;
    let t37092 = F::cast_from(0.20220636637604418766e-5_f64) * t34839;
    let t37108 = F::cast_from(0.12817572129705434851e-5_f64) * t34846 + F::cast_from(0.13259557375557346398e-6_f64) * t34849 - F::cast_from(0.20220636637604418766e-4_f64) * t34851 - F::cast_from(0.69504740211613770836e-3_f64) * t34853 + F::cast_from(0.10298285674687440379e-4_f64) * t34856 - F::cast_from(0.91900712057578208105e-2_f64) * t34858 - F::cast_from(0.2318836277704281739e-4_f64) * t34860 + F::cast_from(0.94685814672924837674e-4_f64) * t34866 - F::cast_from(0.61320337121513228211e-3_f64) * t34868 - F::cast_from(0.84412963981222021456e-7_f64) * t34870;
    (t37080, t37082, t37083, t37084, t37086, t37087, t37088, t37089, t37090, t37091, t37092, t37108)
}
