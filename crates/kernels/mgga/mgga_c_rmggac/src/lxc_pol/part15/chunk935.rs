//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 935/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk935<F: Float>(t10100: F, t236: F, t3352: F, t495: F, t8517: F, t1756: F, t2084: F, t2145: F, t27: F, t1818: F, t3351: F, t40168: F, t498: F, t10018: F, t7255: F, t36674: F, t47570: F, t47572: F, t47577: F, t47581: F, t47585: F, t47588: F, t47594: F, t47596: F, t47598: F, t47600: F, t47602: F, t47607: F) -> (F,) {
    let t47612 = t8517 * t3352 * t236 * t10100 * t495;
    let t47616 = t2145 * t27 * t2084 * t1756;
    let t47621 = t3351 * t40168 * t236 * t1818 * t498;
    let t47623 = t7255 * t10018;
    let t47625 = -0.85129199786595678796e-5 * t47570 - 0.85129199786595678796e-5 * t47572 - 0.12769379967989351819e-4 * t47577 + 0.25538759935978703638e-4 * t47581 - 0.38308139903968055457e-4 * t47585 - 0.2993560425465952141e-1 * t47588 - 0.15243824895787514157e-3 * t36674 - 0.71827762319940103983e-4 * t47594 + 0.17025839957319135759e-4 * t47596 - 0.25538759935978703639e-4 * t47598 + 0.25538759935978703639e-4 * t47600 + 0.85129199786595678796e-5 * t47602 - 0.85129199786595678796e-5 * t47607 + 0.71827762319940103983e-4 * t47612 + 0.90915538847484472429e-2 * t47616 - 0.25538759935978703639e-4 * t47621 - 0.42564599893297839398e-5 * t47623;
    (t47625,)
}
