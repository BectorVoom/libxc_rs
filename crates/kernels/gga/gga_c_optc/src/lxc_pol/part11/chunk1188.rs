//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1188/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1188<F: Float>(t1235: F, t16231: F, t19: F, t56775: F, t1: F, t11368: F, t123: F, t14578: F, t14585: F, t25424: F, t25427: F, t25877: F, t25883: F, t2596: F, t2672: F, t287: F, t297: F, t323: F, t3907: F, t42743: F, t42785: F, t49896: F, t51645: F, t51780: F, t51785: F, t51788: F, t51790: F, t51819: F, t51824: F, t51827: F, t51903: F, t52016: F, t56722: F, t56877: F, t57530: F, t894: F, t913: F, t914: F, t953: F) -> (F, F) {
    let t57857 = t16231 * t1235;
    let t57864 = t56775 * t19;
    let t57897 = 0.18583473745796456084e3 * t51780 - 0.17581974682482873924e4 * t51785 - 0.23967961564076583027e5 * t51788 + 0.59710464543246456043e-1 * t51790 + 0.33587136305576131525e-1 * t953 * t894 * t2596 * t57857 - 0.30909018630360027928e0 * t51819 + 0.12020173911806677527e0 * t51824 + 0.10508593825783314861e7 * t25877 * t323 * t57864 * t2672 - 0.75061384469880820436e5 * t25883 * t323 * t57864 * t297 - 0.41296608323992124631e2 * t51827 + 0.11360101276506094136e1 * t913 * t914 * t287 * t57530 * t297 - 0.90880810212048753088e1 * t11368 * t14585 * t56877 + 0.6058720680803250206e1 * t11368 * t14578 * t56722 + 0.9291736872898228042e2 * t3907 * t49896 * t51645 * t1 - 0.30972456242994093474e2 * t42743 + 0.31957282085435444036e5 * t25424 * t52016 * t25427 * t1235 * t123 + 0.1559479530529405812e2 * t51903 + 0.10324152080998031158e2 * t42785;
    (t57857, t57897)
}
