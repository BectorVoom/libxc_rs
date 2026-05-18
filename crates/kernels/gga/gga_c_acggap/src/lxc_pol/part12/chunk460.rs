//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 460/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk460<F: Float>(t2176: F, t323: F, t1968: F, t1970: F, t1986: F, t1989: F, t1995: F, t1999: F, t2010: F, t2013: F, t2017: F, t2021: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2178 = F::new(0.65854491829355115987e0) * t2176 * t323;
    let t2179 = F::new(0.18868855373762491241e-2) * t1968;
    let t2180 = F::new(0.12862205435420921092e-2) * t1970;
    let t2182 = F::new(0.14291339372689912324e-3) * t1986;
    let t2183 = F::new(0.31448092289604152069e-3) * t1989;
    let t2184 = F::new(0.20965394859736101379e-3) * t1995;
    let t2185 = F::new(0.85748036236139473944e-3) * t1999;
    let t2189 = F::new(0.40015750243531754507e-2) * t2010;
    let t2190 = F::new(0.85748036236139473944e-3) * t2013;
    let t2191 = F::new(0.28015625e-1) * t2017;
    let t2192 = F::new(7.0) / F::new(144.0) * t2021;
    (t2178, t2179, t2180, t2182, t2183, t2184, t2185, t2189, t2190, t2191, t2192)
}
