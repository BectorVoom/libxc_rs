//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1234/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1234<F: Float>(t2723: F, t7266: F, t2586: F, t8048: F, t953: F, t10925: F, t8128: F, t1: F, t11326: F, t11473: F, t11493: F, t11495: F, t24464: F, t24470: F, t24474: F, t24478: F, t25381: F, t25401: F, t25440: F, t25445: F, t25453: F, t25454: F, t25458: F, t25468: F, t25472: F, t2606: F, t2642: F, t2671: F, t2672: F, t2721: F, t2722: F, t2812: F, t2813: F, t3608: F, t3884: F, t3917: F, t7452: F, t7491: F, t7494: F, t7983: F, t7987: F, t8127: F, t875: F, t896: F) -> (F, F) {
    let t25479 = t7266 * t2723;
    let t25492 = t953 * t2586 * t8048;
    let t25494 = t8128 * t10925;
    let t25498 = -F::cast_from(0.17581974682482873924e4_f64) * t11326 * t25440 * t875 * t7452 - F::cast_from(0.93568771831764348721e2_f64) * t11473 * t2642 * t25445 - F::cast_from(0.61944912485988186948e2_f64) * t7491 * t24478 * t1 * t7983 - F::cast_from(0.23967961564076583027e5_f64) * t25453 * t25454 * t7494 + F::cast_from(0.26631068404529536697e4_f64) * t25458 * t25454 * t7452 + F::cast_from(0.17581974682482873924e4_f64) * t3917 * t7987 * t24464 - F::cast_from(0.8790987341241436962e3_f64) * t3884 * t7987 * t24474 + F::cast_from(0.3029360340401625103e1_f64) * t2721 * t3608 * t25468 + F::cast_from(0.23442632909977165232e4_f64) * t3917 * t25472 * t24470 + F::cast_from(0.22720202553012188272e1_f64) * t2721 * t2722 * t25401 + F::cast_from(0.93568771831764348721e2_f64) * t2812 * t2813 * t25479 + F::cast_from(0.18583473745796456084e3_f64) * t11493 * t896 * t2606 * t2672 * t11495 + F::cast_from(0.15146801702008125515e1_f64) * t2721 * t2722 * t25381 - F::cast_from(0.80609127133382715662e-1_f64) * t25492 + F::cast_from(0.71903884692229749079e5_f64) * t8127 * t2671 * t25494;
    (t25479, t25498)
}
