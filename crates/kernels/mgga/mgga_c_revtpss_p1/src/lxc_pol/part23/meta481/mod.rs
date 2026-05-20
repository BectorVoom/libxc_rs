//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta481 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1936;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1937;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1938;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta481<F: Float>(t19456: F, t996: F, t1678: F, t4746: F, t1695: F, t5015: F, t3269: F, t6343: F, t994: F, t19462: F, t378: F, t4772: F, t1079: F, t1096: F, t6258: F, t1000: F, t1073: F, t1076: F, t11201: F, t16302: F, t16362: F, t1652: F, t1680: F, t1696: F, t3047: F, t3063: F, t4743: F, t4752: F, t4935: F, t4947: F, t6235: F, t6259: F, t995: F, t19390: F, t19434: F, t20187: F, t1100: F, t1102: F, t19143: F, t19145: F, t19149: F, t19152: F, t19153: F, t19252: F, t19258: F, t19315: F, t19317: F, t19320: F, t19323: F, t19326: F, t19329: F, t19333: F, t19337: F, t19470: F, t19473: F, t19475: F, t198: F, t336: F, t5019: F, t5023: F, t5024: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t20188, t20191, t20194, t20195, t20204) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1936::<F>(t19456, t996, t1678, t4746, t1695, t5015, t3269, t6343, t994);
        let (t20211, t20214, t20215, t20218, t20219, t20228) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1937::<F>(t19462, t378, t1695, t4772, t1079, t1096, t6258, t1000, t1073, t1076, t11201, t16302, t16362, t1652, t1680, t1696, t20188, t20191, t20195, t20204, t3047, t3063, t4743, t4752, t4935, t4947, t6235, t6259, t995);
        let (t20230, t20234) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1938::<F>(t19390, t19434, t20187, t20228, t1100, t1102, t19143, t19145, t19149, t19152, t19153, t19252, t19258, t19315, t19317, t19320, t19323, t19326, t19329, t19333, t19337, t19470, t19473, t19475, t198, t336, t5019, t5023, t5024);
    (t20188, t20191, t20194, t20195, t20204, t20211, t20214, t20215, t20218, t20219, t20230, t20234)
}
