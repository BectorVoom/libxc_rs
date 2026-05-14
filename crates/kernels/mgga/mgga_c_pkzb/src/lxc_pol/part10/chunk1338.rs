//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1338/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1338<F: Float>(t1134: F, t12315: F, t18258: F, t2112: F, t2119: F, t2120: F, t21346: F, t2146: F, t25126: F, t25170: F, t26676: F, t26714: F, t2957: F, t2964: F, t2965: F, t2989: F, t307: F, t3670: F, t3675: F, t3676: F, t3694: F, t6000: F, t7821: F, t7825: F, t786: F, t7884: F, t790: F, t9648: F, t9657: F) -> (F,) {
    let t26741 = 0.13170898365871023197e1 * t3670 * t2120 + 0.52683593463484092788e1 * t1134 * t7825 - 0.79025390195226139182e1 * t1134 * t7821 + 0.26341796731742046394e1 * t307 * t2964 * t7884 - 0.65854491829355115987e0 * t307 * t790 * (t25126 + t25170 + t26676 + t26714) - 0.79025390195226139182e1 * t786 * t9648 + 0.13170898365871023197e1 * t2112 * t3676 + 0.52683593463484092788e1 * t2957 * t2965 + 0.26341796731742046394e1 * t786 * t9657 + 0.15805078039045227836e2 * t307 * t18258 * t3675 * t2119 - 0.15805078039045227836e2 * t21346 * t12315 * t2989 - 0.65854491829355115987e0 * t3670 * t2146 - 0.39512695097613069591e1 * t307 * t6000 * t3694 * t2119;
    (t26741,)
}
