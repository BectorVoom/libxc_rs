//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2636/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2636<F: Float>(t48488: F, t13783: F, t13789: F, t1388: F, t1390: F, t13944: F, t1410: F, t1868: F, t36776: F, t3889: F, t3934: F, t3938: F, t4012: F, t46645: F, t46649: F, t46652: F, t48143: F, t48438: F, t48445: F, t48449: F, t48453: F, t48458: F, t48462: F, t48466: F, t48475: F, t48487: F, t5591: F, t5671: F, t5675: F, t828: F, t9628: F, t9955: F, t9956: F) -> F {
    let t48489 = F::cast_from(0.16262400898971305032e-2_f64) * t48488;
    let t48490 = -F::cast_from(0.15415400852149882894e-1_f64) * t46645 + F::cast_from(0.12862205435420921092e-1_f64) * t1410 * t4012 * t828 * t5591 * t3889 + F::cast_from(0.42874018118069736972e-2_f64) * t1410 * t4012 * t828 * t1868 * t9628 + F::cast_from(0.76230004213927992338e-3_f64) * t48143 - F::cast_from(0.21437009059034868486e-3_f64) * t1388 * t1390 * t828 * t48438 - F::cast_from(0.38115002106963996168e-4_f64) * t48445 - F::cast_from(0.85748036236139473944e-4_f64) * t48449 + F::cast_from(0.21437009059034868486e-4_f64) * t48453 + F::cast_from(0.42874018118069736972e-3_f64) * t48458 - F::cast_from(0.85748036236139473944e-4_f64) * t48462 + F::cast_from(0.30492001685571196934e-4_f64) * t46649 + F::new(455.0) / F::new(216.0) * t46652 - F::cast_from(0.51448821741683684367e-2_f64) * t5671 * t13789 * t48466 * t5675 - F::cast_from(0.12862205435420921092e-1_f64) * t3934 * t13783 * t48466 * t3938 + F::cast_from(0.38586616306262763275e-2_f64) * t5671 * t36776 * t48475 * t5675 - F::cast_from(0.12862205435420921092e-1_f64) * t3934 * t9955 * t13944 * t9956 + t48487 - t48489;
    t48490
}
