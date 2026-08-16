//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta586 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1821;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1822;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1823;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta586(t74130: f64, t74132: f64, t48313: f64, t85986: f64, t40067: f64, t40072: f64, t47096: f64, t47098: f64, t47109: f64, t47116: f64, t47118: f64, t47122: f64, t48324: f64, t187: f64, t92011: f64, t48331: f64, t48333: f64, t48335: f64, t40076: f64, t40079: f64, t47124: f64, t47131: f64, t47138: f64, t47140: f64, t47142: f64, t47152: f64, t1394: f64, t1877: f64, t1879: f64, t22229: f64, t22236: f64, t225: f64, t22809: f64, t22936: f64, t22944: f64, t22947: f64, t22950: f64, t4049: f64, t47171: f64, t539: f64, t541: f64, t5650: f64, t5651: f64, t6816: f64, t6832: f64, t6837: f64, t6840: f64, t91826: f64, t91870: f64, t91875: f64, t91957: f64, t91964: f64, t91967: f64, t91971: f64, t91981: f64, t92017: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t92019, t92020, t92021, t92022, t92023) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1821(t74130, t74132, t48313, t85986, t40067, t40072, t47096, t47098, t47109, t47116, t47118, t47122);
        let (t92024, t92026, t92027, t92028, t92029, t92030) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1822(t48324, t187, t92011, t48331, t48333, t48335, t40076, t40079, t47124, t47131, t47138, t47140, t47142, t47152);
        let t92063 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1823(t1394, t1877, t1879, t22229, t22236, t225, t22809, t22936, t22944, t22947, t22950, t4049, t47171, t539, t541, t5650, t5651, t6816, t6832, t6837, t6840, t91826, t91870, t91875, t91957, t91964, t91967, t91971, t91981, t92017, t92023, t92030);
    (t92019, t92020, t92021, t92022, t92024, t92026, t92027, t92028, t92029, t92063)
}
