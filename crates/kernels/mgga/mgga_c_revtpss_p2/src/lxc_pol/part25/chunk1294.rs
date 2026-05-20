//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1294/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1294<F: Float>(t27668: F, t995: F, t4982: F, t988: F, t25610: F, t25460: F, t3057: F, t25698: F, t378: F, t8521: F, t1043: F, t1089: F, t1096: F, t11901: F, t12043: F, t12174: F, t16573: F, t19502: F, t1976: F, t25464: F, t25473: F, t25480: F, t25593: F, t25605: F, t25620: F, t25629: F, t25651: F, t25692: F, t25700: F, t27640: F, t27652: F, t27664: F, t3067: F, t3270: F, t7135: F, t7140: F, t7144: F, t7145: F, t7146: F, t7159: F, t7160: F, t93892: F, t93974: F, t93989: F) -> F {
    let t94080 = t995 * t27668;
    let t94081 = t4982 * t988;
    let t94085 = t25610 * t27668;
    let t94095 = t3057 * t25460;
    let t94121 = t25698 * t378;
    let t94122 = t94121 * t8521;
    let t94131 = -F::cast_from(0.78062653693846795158e1_f64) * t7159 * t25464 * t7135 * t3270 - F::cast_from(0.52041769129231196772e1_f64) * t94080 * t93892 * t94081 + F::cast_from(0.52041769129231196772e1_f64) * t94085 * t93892 * t19502 + F::cast_from(0.13010442282307799193e1_f64) * t27640 * t93989 * t16573 - F::cast_from(0.52041769129231196772e1_f64) * t25629 * t93974 * t27652 + F::cast_from(0.10408353825846239354e2_f64) * t94095 * t25593 + F::cast_from(0.39512695097613069591e1_f64) * t25692 * t3067 - F::cast_from(0.15612530738769359031e2_f64) * t7144 * t25464 * t7146 * t3270 - F::cast_from(0.65854491829355115987e0_f64) * t7140 * t12174 + F::cast_from(0.39512695097613069591e1_f64) * t25651 * t12043 + F::cast_from(0.52041769129231196772e1_f64) * t25605 * t93974 * t27664 + F::cast_from(0.26020884564615598386e1_f64) * t25473 * t25480 + F::cast_from(0.52041769129231196772e1_f64) * t7144 * t7160 * t25620 * t1096 - F::cast_from(0.26020884564615598386e1_f64) * t25629 * t25620 * t1043 * t1089 - F::cast_from(0.78062653693846795158e1_f64) * t94122 * t25700 * t1043 * t1089 - F::cast_from(0.8673628188205199462e0_f64) * t7144 * t7145 * t1976 * t11901;
    t94131
}
