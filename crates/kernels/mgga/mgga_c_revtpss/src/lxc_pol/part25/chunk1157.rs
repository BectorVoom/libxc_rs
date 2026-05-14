//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1157/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1157<F: Float>(t3057: F, t7135: F, t11200: F, t1976: F, t3063: F, t8521: F, t7143: F, t1035: F, t1983: F, t36870: F, t1096: F, t19482: F, t25625: F, t1043: F, t1089: F, t11173: F, t11184: F, t11203: F, t25470: F, t25473: F, t25476: F, t25479: F, t25591: F, t25605: F, t25616: F, t25621: F, t25631: F, t25700: F, t27664: F, t3060: F, t3325: F, t7102: F, t7144: F, t7145: F, t7146: F, t7151: F, t7152: F, t7160: F, t7162: F, t93892: F, t93945: F, t988: F) -> (F,) {
    let t94023 = t3057 * t7135;
    let t94026 = t11200 * t1976;
    let t94042 = t3063 * t8521;
    let t94053 = t11200 * t7143;
    let t94063 = t1983 * t36870 * t1035;
    let t94064 = t19482 * t1096;
    let t94068 = t25625 * t7143;
    let t94075 = 0.39512695097613069591e1 * t94023 * t3060 - 0.39512695097613069591e1 * t94026 * t11203 + 0.8673628188205199462e0 * t7151 * t7145 * t1976 * t11173 + 0.52041769129231196772e1 * t7144 * t7160 * t7146 * t3325 - 0.26020884564615598386e1 * t25476 * t25621 + 0.26020884564615598386e1 * t25605 * t93945 * t27664 - 0.52041769129231196772e1 * t94042 * t25631 + 0.10408353825846239354e2 * t25591 * t7145 * t25616 * t988 - 0.52041769129231196772e1 * t7151 * t7160 * t7152 * t3325 - 0.15612530738769359031e2 * t94053 * t7145 * t25700 * t988 + 0.19756347548806534796e1 * t7102 * t11184 + 0.52041769129231196772e1 * t25473 * t25470 - 0.26020884564615598386e1 * t94063 * t93892 * t94064 + 0.52041769129231196772e1 * t94068 * t7162 + 0.26020884564615598386e1 * t25605 * t25479 * t1043 * t1089;
    (t94075,)
}
