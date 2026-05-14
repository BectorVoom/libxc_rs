//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 910/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk910<F: Float>(t1355: F, t22849: F, t22856: F, t1354: F, t1992: F, t1291: F, t1303: F, t140: F, t1993: F, t22726: F, t22846: F, t23806: F, t23812: F, t23828: F, t23835: F, t23839: F, t23842: F, t23847: F, t23850: F, t23855: F, t23857: F, t23866: F, t23869: F, t543: F, t5785: F, t8895: F) -> (F, F, F, F) {
    let t23874 = t1355 * t22849;
    let t23877 = 0.11113000182098765433e-1 * t1355 * t22856;
    let t23878 = t1992 * t1354;
    let t23881 = -0.4833552354146973393e0 * t23839 * t23835 - 0.4833552354146973393e0 * t23842 * t23828 - 0.45306850413028723348e0 * t1993 * t1291 - 0.90613700826057446696e0 * t23847 * t23850 - 0.18611243628760286395e2 * t543 * t22726 - 0.61104346057464762978e-1 * t23855 * t23857 + 0.45306850413028723348e0 * t8895 * t1291 + 0.93056218143801431977e1 * t140 * t22726 + 0.45306850413028723348e0 * t5785 * t23806 + 0.43791161479435967988e1 * t23866 * t23812 + 0.90613700826057446696e0 * t23869 * t23850 - 0.48897200801234567903e0 * t1355 * t22846 + 0.88904001456790123461e-1 * t23874 + t23877 - 0.10001700163888888889e0 * t23878 * t1303;
    (t23874, t23877, t23878, t23881)
}
