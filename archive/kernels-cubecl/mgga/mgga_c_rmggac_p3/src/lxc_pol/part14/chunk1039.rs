//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1039/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1039<F: Float>(t1685: F, t2127: F, t36505: F, t36506: F, t36508: F, t41605: F, t41607: F, t41610: F, t41614: F, t41616: F, t41620: F, t41627: F, t41631: F, t41635: F, t41637: F, t41639: F, t41641: F, t6473: F, t72: F, t7772: F) -> F {
    let t41645 = -t41605 - F::cast_from(0.72042316457491791906e-3_f64) * t41607 - F::cast_from(0.72042316457491791906e-3_f64) * t41610 - t41614 - F::cast_from(0.72042316457491791906e-3_f64) * t41616 - t41620 + F::cast_from(2.0_f64) * t72 * t1685 * t2127 + F::cast_from(0.1064114997332445985e-4_f64) * t41627 - F::cast_from(0.23948483403727617128e0_f64) * t6473 * t7772 - F::cast_from(0.27274661654245341728e-1_f64) * t41631 - F::cast_from(0.40911992481368012592e0_f64) * t41635 - F::cast_from(0.36366215538993788971e0_f64) * t41637 - F::cast_from(0.81823984962736025184e-1_f64) * t41639 + F::cast_from(0.21819729323396273382e0_f64) * t41641 + t36505 + F::cast_from(0.99317399751028291929e-5_f64) * t36506 - F::cast_from(0.66211599834018861286e-4_f64) * t36508;
    t41645
}
