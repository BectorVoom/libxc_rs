//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1007/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1007<F: Float>(t14857: F, t2674: F, t243: F, t4423: F, t231: F, t2662: F, t2661: F, t10722: F, t1565: F, t4352: F, t4366: F, t10726: F, t10868: F, t241: F, t820: F, t10811: F, t4452: F) -> (F, F, F, F, F, F) {
    let t14859 = 0.10164000561857065645e-3 * t2674 * t14857;
    let t14860 = t243 * t4423;
    let t14861 = t14860 * t231;
    let t14862 = t2662 * t14861;
    let t14864 = 0.14291339372689912324e-4 * t2661 * t14862;
    let t14866 = t10722 * t1565;
    let t14868 = t4352 * t4366;
    let t14869 = t10726 * t14868;
    let t14871 = 0.28582678745379824648e-4 * t2661 * t14869;
    let t14894 = t820 * t10868 * t241;
    let t14907 = t10811 * t4452;
    (t14859, t14864, t14866, t14871, t14894, t14907)
}
