//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta656 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2424;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2425;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2426;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2427;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta656<F: Float>(t49222: F, t942: F, t951: F, t959: F, t10524: F, t1580: F, t42110: F, t42113: F, t10723: F, t13658: F, t10526: F, t10623: F, t13659: F, t13732: F, t2940: F, t4483: F, t4489: F, t49278: F, t49280: F, t49282: F, t49426: F, t49485: F, t49488: F, t49491: F, t48760: F, t49256: F, t49259: F, t49262: F, t49268: F, t49271: F, t49273: F, t49276: F, t49530: F, t49563: F, t3041: F, t607: F, t1023: F, t3120: F, t10390: F, t14501: F, t10422: F, t13761: F, t3070: F, t1020: F, t1021: F, t1031: F, t10413: F, t13941: F, t14093: F, t1539: F, t248: F, t3071: F, t3088: F, t3117: F, t360: F, t378: F, t42514: F, t42518: F, t4342: F, t4347: F, t4616: F, t48670: F, t48674: F, t1615: F, t3040: F, t10403: F, t14214: F, t3030: F, t4552: F, t3032: F, t3129: F, t1022: F, t10408: F, t10937: F, t14174: F, t14207: F, t14211: F, t14212: F, t14220: F, t14222: F, t14235: F, t14491: F, t2244: F, t2250: F, t2770: F, t3114: F, t3123: F, t3134: F, t42483: F, t42508: F, t42530: F, t4337: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t49567, t49572, t49575, t49585) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2424::<F>(t49222, t942, t951, t959, t10524, t1580, t42110, t42113, t10723, t13658, t10526, t10623, t13659, t13732, t2940, t4483, t4489, t49278, t49280, t49282, t49426, t49485, t49488, t49491);
        let t49588 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2425::<F>(t48760, t49256, t49259, t49262, t49268, t49271, t49273, t49276, t49530, t49563, t49567, t49572, t49575, t49585);
        let (t49594, t49599, t49609) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2426::<F>(t3041, t607, t1023, t3120, t10390, t14501, t10422, t13761, t3070, t1020, t1021, t1031, t10413, t13941, t14093, t1539, t248, t3071, t3088, t3117, t360, t378, t42514, t42518, t4342, t4347, t4616, t48670, t48674, t49588);
        let (t49616, t49649, t49650, t49654) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2427::<F>(t1615, t3120, t3040, t10403, t10422, t14214, t3030, t4552, t3032, t3129, t1022, t10408, t10413, t10937, t14174, t14207, t14211, t14212, t14220, t14222, t14235, t14491, t2244, t2250, t2770, t3071, t3114, t3117, t3123, t3134, t42483, t42508, t42530, t4337, t49594);
    (t49567, t49572, t49575, t49585, t49588, t49599, t49609, t49616, t49649, t49650, t49654)
}
