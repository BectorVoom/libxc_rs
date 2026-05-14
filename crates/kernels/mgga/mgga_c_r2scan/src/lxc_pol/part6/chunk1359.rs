//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1359/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1359<F: Float>(t25753: F, t25751: F, t565: F, t6482: F, t20137: F, t6475: F, t7257: F, t20902: F, t20904: F, t20906: F, t20914: F, t20916: F, t20921: F, t20925: F, t25740: F, t25742: F, t25744: F, t25748: F) -> (F,) {
    let t25754 = 0.19043987679069580388e-1 * t25753;
    let t25755 = t565 * t25751;
    let t25756 = t25755 * t6482;
    let t25759 = t6475 * t20137 * t7257;
    let t25761 = -0.38415120233790484326e0 * t20902 + 0.34672886960217074253e0 * t20904 - 0.64025200389650807208e0 * t20906 - t25740 - 0.12713391885412927226e1 * t20914 - 0.34909953929791734801e0 * t25742 - 0.17465477326173296717e-1 * t25744 + 0.34930954652346593433e-1 * t25748 - 0.59329162131926993722e1 * t20916 + t20921 - t25754 - 0.57131963037208741166e-1 * t25756 - t20925 + 0.1713958891116262235e0 * t25759;
    (t25761,)
}
