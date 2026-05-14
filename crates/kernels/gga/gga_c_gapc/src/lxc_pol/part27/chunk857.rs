//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 857/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk857<F: Float>(t11597: F, t2993: F, t3001: F, t1030: F, t3008: F, t11356: F, t9256: F, t11535: F, t11541: F, t11547: F, t11550: F, t11552: F, t11555: F, t11559: F, t11562: F, t11564: F, t11567: F, t11570: F, t11572: F, t11574: F, t11581: F, t11584: F, t11592: F, t11595: F) -> (F, F, F, F) {
    let t11598 = t2993 * t11597;
    let t11599 = t11598 * t3001;
    let t11601 = t1030 * t11597;
    let t11602 = t11601 * t3008;
    let t11604 = t2993 * t11356;
    let t11605 = t11604 * t9256;
    let t11607 = 0.24583187891642252608e-7 * t11535 - 0.11049631146297788665e-7 * t11541 + 0.71141006005012433352e-8 * t11547 - 0.1264887086769121065e-7 * t11550 - 0.57970906942607043474e-5 * t11552 - 0.24583187891642252608e-8 * t11555 + 0.16388791927761501739e-8 * t11559 - 0.28985453471303521737e-5 * t11562 + 0.57970906942607043474e-5 * t11564 + 0.10860115658064651693e-4 * t11567 - 0.11594181388521408695e-4 * t11570 + 0.10860115658064651693e-4 * t11572 - 0.17376185052903442709e-3 * t11574 + 0.14480154210752868924e-5 * t11581 - 0.67471788194444444447e-5 * t11584 + 0.48342136265052825409e-8 * t11592 + 0.45289771048911752714e-7 * t11595 + 0.33765185592488808582e-6 * t11599 + 0.67530371184977617164e-6 * t11602 - 0.10551620497652752682e-7 * t11605;
    (t11598, t11601, t11604, t11607)
}
