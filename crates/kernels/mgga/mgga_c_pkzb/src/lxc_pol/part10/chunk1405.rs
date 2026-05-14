//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1405/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1405<F: Float>(t10055: F, t2380: F, t6475: F, t2402: F, t3860: F, t2407: F, t10258: F, t8406: F, t10251: F, t1238: F, t179: F, t19193: F, t2226: F, t23375: F, t23381: F, t23383: F, t2418: F, t27287: F, t2888: F, t3174: F, t404: F, t8249: F, t8411: F, t8415: F, t932: F) -> (F,) {
    let t28324 = t2380 * t6475 * t10055;
    let t28333 = t3860 * t2402;
    let t28335 = t3860 * t2407;
    let t28345 = t10258 * t8406;
    let t28351 = 0.19055119163586549765e-3 * t19193 - 0.11433071498151929859e-2 * t28324 - 0.57165357490759649296e-3 * t23375 - 0.20325460441158986416e-2 * t23381 + 0.30488190661738479624e-2 * t23383 - t3174 * t2888 * t10251 * t2226 / 16.0 - 0.16090989515917530913e-2 * t28333 - 0.19309187419101037095e-1 * t28335 - 0.14481890564325777821e-1 * t3860 * t2418 + 0.45732285992607719436e-2 * t1238 * t8249 - 0.42874018118069736972e-3 * t404 * t179 * t932 * t27287 - 0.18292914397043087774e-1 * t28345 - 0.27439371595564631662e-1 * t10258 * t8411 - 0.13719685797782315831e-1 * t10258 * t8415;
    (t28351,)
}
