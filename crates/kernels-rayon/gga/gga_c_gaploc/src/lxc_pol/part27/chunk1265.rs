//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1265/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1265(t10820: f64, t10914: f64, t2089: f64, t539: f64, t16036: f64, t6111: f64, t1457: f64, t2103: f64, t32223: f64, t32219: f64, t11065: f64, t5666: f64) -> (f64, f64, f64, f64, f64) {
    let t33409 = 0.28600391961480341335e1_f64 * t10914 * t539 * t2089 * t10820;
    let t33412 = 0.57200783922960682671e1_f64 * t6111 * t16036 * t10820;
    let t33416 = 0.71500979903700853338e0_f64 * t2103 * t1457 * t32223;
    let t33419 = 0.14300195980740170668e1_f64 * t2103 * t1457 * t32219;
    let t33421 = 0.2556195063594716645e1_f64 * t5666 * t11065;
    (t33409, t33412, t33416, t33419, t33421)
}
