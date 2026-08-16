//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1234/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1234(t2723: f64, t7266: f64, t2586: f64, t8048: f64, t953: f64, t10925: f64, t8128: f64, t1: f64, t11326: f64, t11473: f64, t11493: f64, t11495: f64, t24464: f64, t24470: f64, t24474: f64, t24478: f64, t25381: f64, t25401: f64, t25440: f64, t25445: f64, t25453: f64, t25454: f64, t25458: f64, t25468: f64, t25472: f64, t2606: f64, t2642: f64, t2671: f64, t2672: f64, t2721: f64, t2722: f64, t2812: f64, t2813: f64, t3608: f64, t3884: f64, t3917: f64, t7452: f64, t7491: f64, t7494: f64, t7983: f64, t7987: f64, t8127: f64, t875: f64, t896: f64) -> (f64, f64) {
    let t25479 = t7266 * t2723;
    let t25492 = t953 * t2586 * t8048;
    let t25494 = t8128 * t10925;
    let t25498 = -0.17581974682482873924e4_f64 * t11326 * t25440 * t875 * t7452 - 0.93568771831764348721e2_f64 * t11473 * t2642 * t25445 - 0.61944912485988186948e2_f64 * t7491 * t24478 * t1 * t7983 - 0.23967961564076583027e5_f64 * t25453 * t25454 * t7494 + 0.26631068404529536697e4_f64 * t25458 * t25454 * t7452 + 0.17581974682482873924e4_f64 * t3917 * t7987 * t24464 - 0.8790987341241436962e3_f64 * t3884 * t7987 * t24474 + 0.3029360340401625103e1_f64 * t2721 * t3608 * t25468 + 0.23442632909977165232e4_f64 * t3917 * t25472 * t24470 + 0.22720202553012188272e1_f64 * t2721 * t2722 * t25401 + 0.93568771831764348721e2_f64 * t2812 * t2813 * t25479 + 0.18583473745796456084e3_f64 * t11493 * t896 * t2606 * t2672 * t11495 + 0.15146801702008125515e1_f64 * t2721 * t2722 * t25381 - 0.80609127133382715662e-1_f64 * t25492 + 0.71903884692229749079e5_f64 * t8127 * t2671 * t25494;
    (t25479, t25498)
}
