//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1395/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1395<F: Float>(t10010: F, t8694: F, t2294: F, t2582: F, t9994: F, t2122: F, t2124: F, t2139: F, t24475: F, t25323: F, t2567: F, t2625: F, t29635: F, t29670: F, t29674: F, t29680: F, t29692: F, t32896: F, t33196: F, t33244: F, t360: F, t495: F, t5109: F, t6121: F, t7321: F, t7461: F, t8825: F, t9110: F, t9115: F) -> (F, F) {
    let t33815 = t8694 * t10010;
    let t33840 = t2582 * t2294 * t9994;
    let t33849 = 0.13869154784086829701e1 * t29635 + 0.16463622957338778996e0 * t2122 * t7321 * t33815 + 0.39006997830244208535e0 * t2139 * t5109 * t33196 + t25323 - 0.15602799132097683414e1 * t7461 * t360 * t8825 * t2625 + 0.25610080155860322884e1 * t29670 - 0.43341108700271342816e-1 * t2582 * t360 * t32896 * t495 - 0.15602799132097683414e1 * t7461 * t360 * t2567 * t9110 - 0.7801399566048841707e1 * t24475 * t360 * t2567 * t9115 + 0.34672886960217074253e0 * t33840 - 0.38415120233790484326e0 * t29674 - 0.32927245914677557992e0 * t2122 * t2124 * t33244 * t6121 - 0.20803732176130244552e1 * t29680 - 0.69345773920434148506e0 * t29692;
    (t33815, t33849)
}
