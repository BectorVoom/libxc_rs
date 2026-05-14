//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 891/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk891<F: Float>(t42826: F, t42828: F, t42838: F, t42841: F, t42844: F, t42845: F, t42847: F, t42850: F, t47013: F, t47016: F, t47019: F, t47024: F, t47028: F, t47032: F, t47036: F, t47040: F, t47042: F, t47047: F) -> (F,) {
    let t50977 = -t42826 + 0.56910013271352299198e-1 * t47013 + 0.56910013271352299198e-1 * t47016 - t47019 + t42828 + t42838 + t42841 - t42844 + 0.1138200265427045984e0 * t47024 + 0.1138200265427045984e0 * t47028 - t42845 + t42847 + t42850 - 0.63233348079280332442e-2 * t47032 + 0.23712505529730124666e-2 * t47036 + 0.56910013271352299198e-1 * t47040 + 0.31616674039640166221e-2 * t47042 - 0.39837009289946609438e0 * t47047;
    (t50977,)
}
