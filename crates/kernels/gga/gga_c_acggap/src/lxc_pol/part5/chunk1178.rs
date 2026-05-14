//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1178/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1178<F: Float>(t119: F, t6413: F, t5511: F, t868: F, t852: F, t1914: F, t3874: F, t5384: F, t871: F, t5378: F, t5385: F, t1215: F, t1221: F, t14695: F, t150: F, t187: F, t18872: F, t18875: F, t1937: F, t24340: F, t3875: F, t446: F, t464: F, t6558: F) -> (F,) {
    let t24516 = t119 * t6413;
    let t24519 = t868 * t5511;
    let t24521 = t852 * t5511;
    let t24531 = t5384 * t3874 * t1914 * t871;
    let t24534 = t5384 * t5385 * t5378;
    let t24540 = 0.13170898365871023197e1 * t18872 + 0.26341796731742046394e1 * t18875 - 0.13170898365871023197e1 * t24516 * t464 - t14695 + 0.13170898365871023197e1 * t24519 + 0.13170898365871023197e1 * t24521 - 0.13170898365871023197e1 * t1215 * t6558 + 0.65854491829355115987e0 * t119 * t24340 * t150 * t187 + 0.79025390195226139182e1 * t24531 - 0.52683593463484092788e1 * t24534 - 0.39512695097613069591e1 * t446 * t3875 * t1937 * t1221;
    (t24540,)
}
