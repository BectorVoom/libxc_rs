//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1180/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1180<F: Float>(t10991: F, t11018: F, t1891: F, t23658: F, t24410: F, t24412: F, t24418: F, t24420: F, t24427: F, t24431: F, t24434: F, t24438: F, t24443: F, t24448: F, t24458: F, t24459: F, t24464: F, t24470: F, t24474: F, t24478: F, t24480: F, t2596: F, t2640: F, t2642: F, t2643: F, t2645: F, t2648: F, t2673: F, t2813: F, t3835: F, t6534: F, t7349: F, t7350: F, t7449: F, t7451: F, t7460: F, t7470: F, t7481: F, t7488: F, t7491: F, t8114: F, t8134: F, t875: F) -> F {
    let t24484 = -F::cast_from(0.5680050638253047068e0_f64) * t10991 * t2648 * t1891 * t7470 + F::cast_from(0.47333755318775392234e0_f64) * t10991 * t2596 * t1891 * t7470 + F::cast_from(0.3029360340401625103e1_f64) * t7488 * t7350 - F::cast_from(0.63111673758367189645e-1_f64) * t24410 + F::cast_from(0.5680050638253047068e0_f64) * t2640 * t2642 * t24412 - F::cast_from(0.37867004255020313788e0_f64) * t24418 + F::cast_from(0.95929744112718128262e1_f64) * t24420 * t2645 - F::cast_from(0.2840025319126523534e0_f64) * t2640 * t7460 * t7349 - F::cast_from(0.94667510637550784466e0_f64) * t2640 * t7481 * t875 * t24427 + F::cast_from(0.28345352648723563784e5_f64) * t8134 * t24431 * t24434 - F::cast_from(0.28345352648723563785e5_f64) * t8114 * t24431 * t24438 + F::cast_from(0.47242254414539272975e4_f64) * t11018 * t24431 * t24443 + F::cast_from(0.42074449172244793095e0_f64) * t2640 * t24448 * t6534 * t875 * t2643 - F::cast_from(0.14488602482981263091e-1_f64) * t3835 * t2813 * t23658 - F::cast_from(0.30524261601532767229e2_f64) * t7449 * t24458 * t24459 + F::cast_from(0.36629113921839320676e2_f64) * t7491 * t7451 * t24464 + F::cast_from(0.61048523203065534458e2_f64) * t7491 * t24458 * t24470 - F::cast_from(0.18314556960919660338e2_f64) * t7449 * t7451 * t24474 + F::cast_from(0.73258227843678641352e2_f64) * t7491 * t24478 * t2673 * t24480;
    t24484
}
