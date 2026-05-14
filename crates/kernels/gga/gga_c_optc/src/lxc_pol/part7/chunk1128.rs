//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1128/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1128<F: Float>(t288: F, t7835: F, t2586: F, t8057: F, t940: F, t8149: F, t8153: F, t530: F, t864: F, t2721: F, t2724: F, t2723: F, t7299: F, t11368: F, t11451: F, t11455: F, t18634: F, t23825: F, t25063: F, t25117: F, t25365: F, t25445: F, t2591: F, t2598: F, t2722: F, t2812: F, t3836: F, t3917: F, t3918: F, t7870: F, t8187: F, t8203: F, t8220: F, t894: F, t914: F, t930: F, t953: F) -> (F, F, F) {
    let t25610 = t288 * t7835;
    let t25618 = t940 * t2586 * t8057;
    let t25620 = t8149 * t8153;
    let t25622 = t530 * t864;
    let t25624 = t2721 * t25622 * t2724;
    let t25633 = t2723 * t7299;
    let t25646 = -0.28131159491972598279e5 * t11451 * t8203 + 0.14065579745986299139e5 * t11455 * t8187 + 0.35163949364965747848e4 * t3917 * t25610 * t3918 - 0.90880810212048753088e1 * t11368 * t18634 * t25445 + 0.1559479530529405812e2 * t25618 - 0.80782942410710002746e1 * t25620 - 0.10097867801338750343e1 * t25624 + 0.15146801702008125515e1 * t2721 * t2722 * t25365 - 0.30228422675018518374e0 * t953 * t894 * t7870 * t23825 - 0.1559479530529405812e3 * t2812 * t3836 * t25633 + 0.10210489436895143984e1 * t8220 * t2591 + 0.17017482394825239973e1 * t8220 * t2598 + 0.11590881986385010473e0 * t930 * t914 * t25117 + 0.25190352229182098644e-1 * t953 * t25063;
    (t25610, t25633, t25646)
}
