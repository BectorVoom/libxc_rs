//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2636/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2636(t48488: f64, t13783: f64, t13789: f64, t1388: f64, t1390: f64, t13944: f64, t1410: f64, t1868: f64, t36776: f64, t3889: f64, t3934: f64, t3938: f64, t4012: f64, t46645: f64, t46649: f64, t46652: f64, t48143: f64, t48438: f64, t48445: f64, t48449: f64, t48453: f64, t48458: f64, t48462: f64, t48466: f64, t48475: f64, t48487: f64, t5591: f64, t5671: f64, t5675: f64, t828: f64, t9628: f64, t9955: f64, t9956: f64) -> f64 {
    let t48489 = 0.16262400898971305032e-2_f64 * t48488;
    let t48490 = -0.15415400852149882894e-1_f64 * t46645 + 0.12862205435420921092e-1_f64 * t1410 * t4012 * t828 * t5591 * t3889 + 0.42874018118069736972e-2_f64 * t1410 * t4012 * t828 * t1868 * t9628 + 0.76230004213927992338e-3_f64 * t48143 - 0.21437009059034868486e-3_f64 * t1388 * t1390 * t828 * t48438 - 0.38115002106963996168e-4_f64 * t48445 - 0.85748036236139473944e-4_f64 * t48449 + 0.21437009059034868486e-4_f64 * t48453 + 0.42874018118069736972e-3_f64 * t48458 - 0.85748036236139473944e-4_f64 * t48462 + 0.30492001685571196934e-4_f64 * t46649 + 455.0_f64 / 216.0_f64 * t46652 - 0.51448821741683684367e-2_f64 * t5671 * t13789 * t48466 * t5675 - 0.12862205435420921092e-1_f64 * t3934 * t13783 * t48466 * t3938 + 0.38586616306262763275e-2_f64 * t5671 * t36776 * t48475 * t5675 - 0.12862205435420921092e-1_f64 * t3934 * t9955 * t13944 * t9956 + t48487 - t48489;
    t48490
}
