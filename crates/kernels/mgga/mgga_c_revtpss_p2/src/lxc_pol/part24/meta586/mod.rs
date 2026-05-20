//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta586 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1821;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1822;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1823;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta586<F: Float>(t74130: F, t74132: F, t48313: F, t85986: F, t40067: F, t40072: F, t47096: F, t47098: F, t47109: F, t47116: F, t47118: F, t47122: F, t48324: F, t187: F, t92011: F, t48331: F, t48333: F, t48335: F, t40076: F, t40079: F, t47124: F, t47131: F, t47138: F, t47140: F, t47142: F, t47152: F, t1394: F, t1877: F, t1879: F, t22229: F, t22236: F, t225: F, t22809: F, t22936: F, t22944: F, t22947: F, t22950: F, t4049: F, t47171: F, t539: F, t541: F, t5650: F, t5651: F, t6816: F, t6832: F, t6837: F, t6840: F, t91826: F, t91870: F, t91875: F, t91957: F, t91964: F, t91967: F, t91971: F, t91981: F, t92017: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t92019, t92020, t92021, t92022, t92023) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1821::<F>(t74130, t74132, t48313, t85986, t40067, t40072, t47096, t47098, t47109, t47116, t47118, t47122);
        let (t92024, t92026, t92027, t92028, t92029, t92030) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1822::<F>(t48324, t187, t92011, t48331, t48333, t48335, t40076, t40079, t47124, t47131, t47138, t47140, t47142, t47152);
        let t92063 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1823::<F>(t1394, t1877, t1879, t22229, t22236, t225, t22809, t22936, t22944, t22947, t22950, t4049, t47171, t539, t541, t5650, t5651, t6816, t6832, t6837, t6840, t91826, t91870, t91875, t91957, t91964, t91967, t91971, t91981, t92017, t92023, t92030);
    (t92019, t92020, t92021, t92022, t92024, t92026, t92027, t92028, t92029, t92063)
}
