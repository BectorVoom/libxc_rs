//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1396/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1396<F: Float>(t27251: F, t27403: F, t27440: F, t27475: F, t27510: F, t27957: F, t27983: F, t28013: F, t2367: F, t3886: F, t5939: F, t10251: F, t300: F, t10047: F, t10213: F, t10214: F, t19026: F, t19030: F, t23075: F, t2380: F, t2381: F, t2383: F, t2395: F, t2396: F, t2411: F, t27062: F, t27226: F, t27232: F, t27234: F, t27237: F, t3199: F, t3206: F, t3913: F, t405: F, t406: F, t6483: F, t758: F, t7984: F, t7988: F, t8256: F, t918: F, t921: F) -> (F, F) {
    let t28016 = t27251 + t27403 + t27440 + t27475 + t27510 + t27957 + t27983 + t28013;
    let t28023 = t2367 * t5939 * t3886;
    let t28033 = t300 * t10251;
    let t28037 = -0.91464571985215438872e-2 * t10047 * t8256 - 5.0 / 648.0 * t19026 + t19030 / 432.0 + 0.51448821741683684368e-2 * t23075 * t406 * t27062 * t27226 - 0.57165357490759649296e-3 * t27232 + 0.5081365110289746604e-3 * t27234 - 0.42874018118069736972e-3 * t2395 * t758 * t27237 * t2396 + 0.42874018118069736972e-3 * t3206 * t2381 * t3913 * t6483 + 0.51448821741683684366e-2 * t2380 * t300 * t2411 * t3199 * t10214 + 0.21437009059034868486e-3 * t918 * t758 * t405 * t28016 * t921 - 0.95275595817932748827e-4 * t28023 + 0.51448821741683684366e-2 * t2380 * t10213 * t921 * t7988 + 0.25724410870841842183e-2 * t2380 * t10213 * t921 * t7984 + 0.25724410870841842184e-2 * t2380 * t28033 * t2383;
    (t28016, t28037)
}
