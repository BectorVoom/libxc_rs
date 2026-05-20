//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta336 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1173;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1174;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1175;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1176;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1177;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta336<F: Float>(t1579: F, t6071: F, t2770: F, t6048: F, t11008: F, t10987: F, t11017: F, t11040: F, t15018: F, t15063: F, t1580: F, t18798: F, t18800: F, t18806: F, t18812: F, t18815: F, t18822: F, t18826: F, t18828: F, t865: F, t23400: F, t10566: F, t10568: F, t10577: F, t10582: F, t10584: F, t10586: F, t1583: F, t18865: F, t1940: F, t198: F, t207: F, t23186: F, t23189: F, t892: F, t9514: F, t9517: F, t9521: F, t6079: F, t10592: F, t10596: F, t10604: F, t10611: F, t11064: F, t23191: F, t23193: F, t23213: F, t23215: F, t23218: F, t23220: F, t23223: F, t9524: F, t9542: F, t23105: F, t23152: F, t4724: F, t6206: F, t981: F, t4719: F, t6227: F, t1633: F, t6189: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t23403, t23404, t23413, t23414, t23420) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1173::<F>(t1579, t6071, t2770, t6048, t11008, t10987, t11017, t11040, t15018, t15063, t1580, t18798, t18800, t18806, t18812, t18815, t18822, t18826, t18828, t865);
        let (t23421, t23428) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1174::<F>(t23400, t23420, t10566, t10568, t10577, t10582, t10584, t10586, t1583, t18865, t1940, t198, t207, t23186, t23189, t892, t9514, t9517, t9521);
        let (t23429, t23434) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1175::<F>(t1583, t6079, t10592, t10596, t10604, t10611, t11064, t198, t207, t23191, t23193, t23213, t23215, t23218, t23220, t23223, t9524, t9542);
        let t23436 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1176::<F>(t23105, t23152, t23428, t23434);
        let (t23446, t23448, t23450, t23451) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1177::<F>(t4724, t6206, t981, t4719, t6227, t1633, t6189);
    (t23403, t23404, t23413, t23414, t23421, t23429, t23436, t23446, t23448, t23450, t23451)
}
