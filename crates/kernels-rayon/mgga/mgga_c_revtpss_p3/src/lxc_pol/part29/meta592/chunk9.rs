//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1976/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1976(t5722: f64, t96576: f64, t28780: f64, t94890: f64, t2435: f64, t28825: f64, t14079: f64, t26265: f64, t98108: f64, t98128: f64, t98130: f64, t98110: f64, t98112: f64, t98116: f64, t98118: f64, t98120: f64, t98122: f64, t98124: f64, t98126: f64, t98132: f64) -> (f64, f64, f64, f64, f64) {
    let t102453 = 0.19514881078765566038e-1_f64 * t96576 * t5722;
    let t102458 = 0.28912093960683998208e-1_f64 * t94890 * t28780;
    let t102462 = t2435 * t28825;
    let t102465 = 0.19514881078765566038e-1_f64 * t26265 * t14079;
    let t102468 = 0.16006300097412701803e-1_f64 * t98108;
    let t102477 = 0.32012600194825403606e-1_f64 * t98128;
    let t102478 = 0.80031500487063509014e-2_f64 * t98130;
    let t102480 = -t102468 + 0.17149607247227894789e-2_f64 * t98110 + 0.68598428988911579156e-2_f64 * t98112 - 0.51448821741683684367e-2_f64 * t98116 + 0.51448821741683684367e-2_f64 * t98118 + 0.34299214494455789578e-2_f64 * t98120 - 0.13719685797782315831e-1_f64 * t98122 - 0.34299214494455789578e-1_f64 * t98124 - 0.85748036236139473944e-3_f64 * t98126 - t102477 + t102478 - 0.17149607247227894789e-1_f64 * t98132;
    (t102453, t102458, t102462, t102465, t102480)
}
