//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta379 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1442;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1443;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1444;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta379(t4234: f64, t4295: f64, t12850: f64, t12860: f64, t16577: f64, t16578: f64, t16581: f64, t16582: f64, t16588: f64, t16612: f64, t9457: f64, t9469: f64, t9476: f64, t9484: f64, t9496: f64, t9715: f64, t9724: f64, t12946: f64, t12922: f64, t12926: f64, t12934: f64, t16618: f64, t16622: f64, t16623: f64, t16624: f64, t16629: f64, t16631: f64, t16633: f64, t16636: f64, t9726: f64, t9780: f64, t9789: f64, t9863: f64, t9869: f64, t5519: f64, t706: f64, t708: f64, t9871: f64, t13115: f64, t157: f64, t4196: f64, t9880: f64, t13107: f64, t13105: f64, t9793: f64, t9797: f64, t9820: f64, t9824: f64, t9876: f64, t9884: f64, t9887: f64, t9890: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16679, t16684) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1442(t4234, t4295, t12850, t12860, t16577, t16578, t16581, t16582, t16588, t16612, t9457, t9469, t9476, t9484, t9496, t9715, t9724);
        let (t16685, t16686) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1443(t12946, t12922, t12926, t12934, t16618, t16622, t16623, t16624, t16629, t16631, t16633, t16636, t9726, t9780, t9789, t9863);
        let (t16688, t16691, t16692, t16695, t16696, t16697, t16698) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1444(t9869, t5519, t706, t708, t9871, t13115, t157, t4196, t9880, t13107, t13105, t9793, t9797, t9820, t9824, t9876, t9884, t9887, t9890);
    (t16679, t16684, t16685, t16686, t16688, t16691, t16692, t16695, t16696, t16697, t16698)
}
