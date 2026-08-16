//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta481 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1936;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1937;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1938;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta481(t19456: f64, t996: f64, t1678: f64, t4746: f64, t1695: f64, t5015: f64, t3269: f64, t6343: f64, t994: f64, t19462: f64, t378: f64, t4772: f64, t1079: f64, t1096: f64, t6258: f64, t1000: f64, t1073: f64, t1076: f64, t11201: f64, t16302: f64, t16362: f64, t1652: f64, t1680: f64, t1696: f64, t3047: f64, t3063: f64, t4743: f64, t4752: f64, t4935: f64, t4947: f64, t6235: f64, t6259: f64, t995: f64, t19390: f64, t19434: f64, t20187: f64, t1100: f64, t1102: f64, t19143: f64, t19145: f64, t19149: f64, t19152: f64, t19153: f64, t19252: f64, t19258: f64, t19315: f64, t19317: f64, t19320: f64, t19323: f64, t19326: f64, t19329: f64, t19333: f64, t19337: f64, t19470: f64, t19473: f64, t19475: f64, t198: f64, t336: f64, t5019: f64, t5023: f64, t5024: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20188, t20191, t20194, t20195, t20204) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1936(t19456, t996, t1678, t4746, t1695, t5015, t3269, t6343, t994);
        let (t20211, t20214, t20215, t20218, t20219, t20228) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1937(t19462, t378, t1695, t4772, t1079, t1096, t6258, t1000, t1073, t1076, t11201, t16302, t16362, t1652, t1680, t1696, t20188, t20191, t20195, t20204, t3047, t3063, t4743, t4752, t4935, t4947, t6235, t6259, t995);
        let (t20230, t20234) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1938(t19390, t19434, t20187, t20228, t1100, t1102, t19143, t19145, t19149, t19152, t19153, t19252, t19258, t19315, t19317, t19320, t19323, t19326, t19329, t19333, t19337, t19470, t19473, t19475, t198, t336, t5019, t5023, t5024);
    (t20188, t20191, t20194, t20195, t20204, t20211, t20214, t20215, t20218, t20219, t20230, t20234)
}
