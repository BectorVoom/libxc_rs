//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta661 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2314;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2315;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2316;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2317;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2318;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2319;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2320;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta661(t90963: f64, t22633: f64, t26421: f64, t3851: f64, t6976: f64, t26418: f64, t6914: f64, t7736: f64, t80854: f64, t81064: f64, t22704: f64, t22705: f64, t26410: f64, t1336: f64, t1352: f64, t16033: f64, t16055: f64, t1825: f64, t22879: f64, t26404: f64, t26442: f64, t26453: f64, t26458: f64, t3773: f64, t3777: f64, t5234: f64, t5344: f64, t7747: f64, t81199: f64, t90942: f64, t90946: f64, t90952: f64, t90957: f64, t90962: f64, t26432: f64, t6897: f64, t794: f64, t22642: f64, t22690: f64, t26395: f64, t22863: f64, t7737: f64, t26448: f64, t90497: f64, t215: f64, t6916: f64, t225: f64, t3787: f64, t562: f64, t16313: f64, t22751: f64, t26385: f64, t16068: f64, t1992: f64, t81149: f64, t16060: f64, t26403: f64, t3856: f64, t5250: f64, t5334: f64, t6988: f64, t81115: f64, t81125: f64, t81127: f64, t81140: f64, t81147: f64, t81154: f64, t3719: f64, t6637: f64, t6888: f64, t7722: f64, t16307: f64, t90915: f64, t81187: f64, t81197: f64, t1307: f64, t26331: f64, t26446: f64, t90818: f64, t3734: f64, t90591: f64, t22710: f64, t22874: f64, t22877: f64, t26456: f64, t3793: f64, t81160: f64, t81184: f64, t81189: f64, t26389: f64, t22897: f64, t3792: f64, t90870: f64, t26467: f64, t26426: f64, t81046: f64, t7732: f64, t81195: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90964, t90968, t90971, t90980, t90983) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2314(t90963, t22633, t26421, t3851, t6976, t26418, t6914, t7736, t80854, t81064, t22704, t22705, t26410);
        let t90985 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2315(t90983, t1336, t1352, t16033, t16055, t1825, t22879, t26404, t26442, t26453, t26458, t3773, t3777, t3851, t5234, t5344, t7747, t81199, t90942, t90946, t90952, t90957, t90962, t90964, t90968, t90971, t90980);
        let (t90988, t90993, t91000, t91002, t91004) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2316(t26432, t6897, t794, t22642, t22690, t26395, t22863, t7737, t26448, t90497, t215, t6916);
        let (t91005, t91019) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2317(t225, t3787, t562, t16313, t91004, t22751, t26385, t16068, t1992, t6976, t81149, t16060, t26403, t3856, t5250, t5334, t5344, t6988, t81115, t81125, t81127, t81140, t81147, t81154, t90942, t90988, t90993, t91000, t91002);
        let (t91025, t91029, t91036, t91043, t91045, t91048) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2318(t26395, t3719, t6637, t6888, t3787, t7722, t16307, t90915, t91004, t81187, t81197, t1307, t26331, t26446, t90818);
        let t91059 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2319(t26421, t26446, t3734, t90591, t1336, t22710, t22874, t22877, t26403, t26456, t26458, t3777, t3793, t3851, t3856, t5234, t5250, t5334, t5344, t81160, t81184, t81189, t90946, t91025, t91029, t91036, t91043, t91045, t91048);
        let (t91065, t91074, t91077, t91078, t91081) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2320(t22751, t26389, t1992, t22897, t3792, t90870, t26467, t6914, t26426, t81046, t22690, t7732, t81195);
    (t90985, t91005, t91019, t91059, t91065, t91074, t91077, t91078, t91081)
}
