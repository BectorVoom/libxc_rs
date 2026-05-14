//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1263/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1263<F: Float>(t183: F, t24253: F, t1783: F, t3410: F, t1034: F, t164: F, t167: F, t1717: F, t1721: F, t20474: F, t24050: F, t24105: F, t24151: F, t24753: F, t2647: F, t2670: F, t2682: F, t588: F, t7096: F, t7123: F, t7126: F, t8954: F, t8958: F) -> (F,) {
    let t24845 = t183 * t24253;
    let t24853 = t1783 * t3410;
    let t24869 = -0.26341796731742046394e1 * t588 * t2670 * t2647 + 0.39512695097613069591e1 * t2682 * t24151 + 0.79025390195226139182e1 * t7126 * t8958 - 0.39512695097613069591e1 * t7123 * t24105 + 0.26341796731742046394e1 * t1717 * t24845 * t1721 + 0.65854491829355115987e0 * t167 * t24753 - 0.39512695097613069591e1 * t7123 * t24050 + 0.13170898365871023197e1 * t1717 * t24853 * t1721 - 0.13170898365871023197e1 * t588 * t24845 * t164 - 0.79025390195226139182e1 * t20474 * t8954 - 0.65854491829355115987e0 * t588 * t24853 * t164 - 0.13170898365871023197e1 * t588 * t7096 * t1034 * t164;
    (t24869,)
}
