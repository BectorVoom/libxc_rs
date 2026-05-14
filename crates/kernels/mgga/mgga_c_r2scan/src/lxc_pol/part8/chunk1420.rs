//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1420/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1420<F: Float>(t26147: F, t26151: F, t26175: F, t26178: F, t26180: F, t26183: F, t26191: F, t26193: F, t30626: F, t30630: F, t30633: F, t30635: F, t30639: F, t30645: F, t2155: F, t33246: F) -> (F, F) {
    let t34443 = -t26147 + 0.57131963037208741164e-1 * t26151 - t26175 + t26178 - t26180 - t26183 - 0.24451668256642615404e1 * t26191 + 0.9878173774403267398e-1 * t26193 + 0.58544643236296698112e-1 * t30626 - 0.34930954652346593433e-1 * t30630 - 0.1047928639570397803e0 * t30633 + 0.34930954652346593433e-1 * t30635 + 0.34930954652346593433e-1 * t30639 + 0.34930954652346593433e-1 * t30645;
    let t34458 = t2155 * t33246;
    (t34443, t34458)
}
