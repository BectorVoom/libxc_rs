//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1291/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1291<F: Float>(t11120: F, t3140: F, t1035: F, t1983: F, t1043: F, t1089: F, t11122: F, t11174: F, t11202: F, t12132: F, t1976: F, t25465: F, t25466: F, t25473: F, t25476: F, t25483: F, t25586: F, t25601: F, t25611: F, t25613: F, t25699: F, t27669: F, t3059: F, t4976: F, t7102: F, t7135: F, t7144: F, t7145: F, t7147: F, t7159: F, t7162: F, t93892: F, t93959: F, t93963: F, t93968: F, t93974: F, t93983: F, t93984: F, t93989: F, t93994: F, t94005: F, t988: F) -> F {
    let t94014 = t3140 * t11120;
    let t94016 = t1983 * t94014 * t1035;
    let t94021 = -F::cast_from(0.26020884564615598386e1_f64) * t93959 * t7147 + F::cast_from(0.52041769129231196772e1_f64) * t93963 * t25613 + F::cast_from(0.10408353825846239354e2_f64) * t25476 * t25601 + F::cast_from(0.10408353825846239354e2_f64) * t7159 * t93968 * t1976 * t11122 + F::cast_from(0.52041769129231196772e1_f64) * t25611 * t93974 * t4976 + F::cast_from(0.26020884564615598386e1_f64) * t25611 * t25483 * t1043 * t1089 + F::cast_from(0.52041769129231196772e1_f64) * t93983 * t93892 * t93984 - F::cast_from(0.26020884564615598386e1_f64) * t27669 * t93989 * t12132 + F::cast_from(0.10408353825846239354e2_f64) * t93994 * t7145 * t1976 * t11202 - F::cast_from(0.78062653693846795158e1_f64) * t25699 * t7145 * t7135 * t3059 - F::cast_from(0.78062653693846795158e1_f64) * t25473 * t25466 + F::cast_from(0.26020884564615598386e1_f64) * t94005 * t7162 - F::cast_from(0.26020884564615598386e1_f64) * t7144 * t7145 * t25586 * t988 - F::cast_from(0.65854491829355115987e0_f64) * t7102 * t11174 - F::cast_from(0.78062653693846795158e1_f64) * t94016 * t25465 * t1043 * t1089;
    t94021
}
