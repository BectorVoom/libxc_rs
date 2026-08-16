//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3326/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3326<F: Float>(t14480: F, t252: F, t2782: F, t4533: F, t14991: F, t50208: F, t14485: F, t14987: F, t18657: F, t213: F, t14983: F, t10513: F, t11008: F, t15038: F, t18313: F, t18663: F, t18784: F, t18785: F, t2765: F, t2770: F, t2828: F, t41115: F, t41118: F, t41125: F, t4474: F, t51746: F, t51750: F, t51756: F, t51759: F, t6048: F, t6049: F, t6072: F, t865: F, t886: F, t887: F) -> F {
    let t63091 = t2782 * t252 * t14480 * t4533;
    let t63094 = t50208 * t14991;
    let t63099 = t14987 * t14485;
    let t63103 = t213 * t18657;
    let t63109 = t14987 * t14983;
    let t63129 = -F::cast_from(0.43902994552903410656e-1_f64) * t63091 + F::cast_from(0.13009920719177044025e-1_f64) * t41115 + F::cast_from(0.78059524315062264149e-1_f64) * t63094 - F::cast_from(0.19514881078765566038e-1_f64) * t51746 - F::cast_from(0.65854491829355115987e0_f64) * t10513 * t6072 + F::cast_from(0.26019841438354088049e-1_f64) * t63099 + F::cast_from(0.26341796731742046394e1_f64) * t4474 * t15038 - F::cast_from(0.13170898365871023197e1_f64) * t63103 * t887 - F::cast_from(0.21951497276451705328e-1_f64) * t51750 + F::cast_from(0.13170898365871023197e1_f64) * t10513 * t6049 - F::cast_from(0.39029762157531132074e-1_f64) * t63109 + F::cast_from(0.22089088168956307394e-3_f64) * t41118 - F::cast_from(0.39512695097613069591e1_f64) * t865 * t11008 * t6048 * t2828 - F::cast_from(0.79025390195226139182e1_f64) * t2765 * t18663 + F::cast_from(0.26341796731742046394e1_f64) * t865 * t2770 * t18784 * t886 - F::cast_from(0.13170898365871023197e1_f64) * t2765 * t18785 + F::cast_from(0.65049603595885220126e-3_f64) * t41125 + F::cast_from(0.52683593463484092788e1_f64) * t2765 * t18313 - F::cast_from(0.520396828767081761e-2_f64) * t51756 + F::cast_from(0.21951497276451705328e-1_f64) * t51759;
    t63129
}
