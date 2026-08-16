//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta580 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2196;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2197;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2198;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2199;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2200;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta580(t1579: f64, t6071: f64, t2770: f64, t6048: f64, t11008: f64, t10987: f64, t11017: f64, t11040: f64, t15018: f64, t15063: f64, t1580: f64, t18798: f64, t18800: f64, t18806: f64, t18812: f64, t18815: f64, t18822: f64, t18826: f64, t18828: f64, t865: f64, t23400: f64, t10566: f64, t10568: f64, t10577: f64, t10582: f64, t10584: f64, t10586: f64, t1583: f64, t18865: f64, t1940: f64, t198: f64, t207: f64, t23186: f64, t23189: f64, t892: f64, t9514: f64, t9517: f64, t9521: f64, t6079: f64, t10592: f64, t10596: f64, t10604: f64, t10611: f64, t11064: f64, t23191: f64, t23193: f64, t23213: f64, t23215: f64, t23218: f64, t23220: f64, t23223: f64, t9524: f64, t9542: f64, t23105: f64, t23152: f64, t4724: f64, t6206: f64, t981: f64, t4719: f64, t6227: f64, t1633: f64, t6189: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23403, t23404, t23413, t23414, t23420) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2196(t1579, t6071, t2770, t6048, t11008, t10987, t11017, t11040, t15018, t15063, t1580, t18798, t18800, t18806, t18812, t18815, t18822, t18826, t18828, t865);
        let (t23421, t23428) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2197(t23400, t23420, t10566, t10568, t10577, t10582, t10584, t10586, t1583, t18865, t1940, t198, t207, t23186, t23189, t892, t9514, t9517, t9521);
        let (t23429, t23434) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2198(t1583, t6079, t10592, t10596, t10604, t10611, t11064, t198, t207, t23191, t23193, t23213, t23215, t23218, t23220, t23223, t9524, t9542);
        let t23436 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2199(t23105, t23152, t23428, t23434);
        let (t23446, t23448, t23450, t23451) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2200(t4724, t6206, t981, t4719, t6227, t1633, t6189);
    (t23403, t23404, t23413, t23414, t23421, t23429, t23436, t23446, t23448, t23450, t23451)
}
