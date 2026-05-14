//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 930/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk930<F: Float>(t1300: F, t15632: F, t15797: F, t1603: F, t1701: F, t22603: F, t22715: F, t22826: F, t22839: F, t22858: F, t25768: F, t25813: F, t29528: F, t29531: F, t29534: F, t29540: F, t29546: F, t29551: F, t29555: F, t29559: F, t29562: F, t4446: F, t5611: F, t7889: F) -> (F,) {
    let t29567 = 0.14846767889314528222e-3 * t25768 - 0.23254900946437792e-1 * t1603 * t29528 + 0.46509801892875584e-2 * t1603 * t29531 + 0.22227677429409423704e-2 * t1300 * t1701 * t29534 + 0.46509801892875584e-1 * t22826 * t4446 - 0.44455354858818847408e-2 * t7889 * t1701 * t29540 - 0.60102574844279699039e-6 * t15632 * t22839 + 0.38731446812548799881e-3 * t1603 * t29546 + 0.25537443351851851852e-1 * t25813 - 0.6384360837962962963e-2 * t5611 * t29551 - 0.85124811172839506173e-2 * t5611 * t29555 + 0.12768721675925925926e-1 * t5611 * t29559 - 0.51690243689028715488e-4 * t22603 * t29562 + t22858 + 0.13519760450715832853e-3 * t15797 * t22715;
    (t29567,)
}
