//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1278/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1278<F: Float>(t2030: F, t7575: F, t18002: F, t2027: F, t287: F, t17929: F, t17945: F, t17999: F, t18008: F, t2135: F, t22007: F, t25117: F, t2969: F, t2970: F, t2980: F, t5718: F, t5931: F, t5952: F, t7666: F, t7737: F, t7743: F, t7833: F, t7837: F, t7845: F, t7864: F, t7868: F, t9660: F, t9662: F, t9667: F, t9674: F, t9685: F, t9695: F, t9707: F) -> (F, F, F, F) {
    let t25136 = t2030 * t7575;
    let t25147 = t18002 * t2027;
    let t25155 = t287 * t7575;
    let t25170 = 0.52683593463484092788e1 * t7837 * t9667 - 0.65854491829355115987e0 * t17945 * t9660 * t22007 * t7666 + 0.79025390195226139182e1 * t5952 * t25117 * t9662 + 0.26341796731742046394e1 * t2969 * t2970 * t25136 + 0.92196288561097162379e1 * t17929 * t9660 * t22007 * t7743 - 0.65854491829355115987e0 * t9695 * t7864 + 0.15805078039045227836e2 * t17999 * t9660 * t22007 * t25147 - 0.23707617058567841754e2 * t18008 * t9660 * t22007 * t7737 - 0.13170898365871023197e1 * t2980 * t2970 * t25155 + 0.13170898365871023197e1 * t5931 * t9674 * t7868 + 0.65854491829355115987e0 * t9707 * t2135 + 0.39512695097613069591e1 * t5952 * t9685 * t7833 - 0.39512695097613069591e1 * t5718 * t9685 * t7845;
    (t25136, t25147, t25155, t25170)
}
