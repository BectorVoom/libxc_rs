//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 831/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk831<F: Float>(t292: F, t14769: F, t14887: F, t799: F, t27: F, t89: F, t1213: F, t1636: F, t668: F, t848: F, t2999: F, t375: F, t4130: F, t10243: F, t10246: F, t10276: F, t10282: F, t10286: F, t10394: F, t10398: F, t14688: F, t14692: F, t14697: F, t14701: F, t14706: F, t14708: F, t14711: F, t14715: F, t14718: F) -> (F, F, F, F, F, F) {
    let t293 = 0.1e-59 < t292;
    let t14889 = piecewise3(t293, t14769 + t14887, 0.0);
    let t14890 = t799 * t14889;
    let t14892 = t89 * t27 * t14890;
    let t14895 = t89 * t1636 * t1213;
    let t14897 = t848 * t668;
    let t14899 = t89 * t2999 * t14897;
    let t14902 = t89 * t375 * t4130;
    let t14903 = t14902 / 9.0;
    let t14904 = 2.0 / 27.0 * t14688 - 2.0 / 9.0 * t14692 + 2.0 / 3.0 * t14697 + t14701 / 3.0 - t14706 + t10394 / 18.0 - t14708 - t10276 / 9.0 - t10246 / 27.0 - t14711 + t10282 / 54.0 + t10286 / 81.0 - 2.0 / 81.0 * t14715 - 11.0 / 27.0 * t14718 - t10243 / 27.0 - t14892 / 6.0 - 2.0 / 27.0 * t14895 + t14899 / 9.0 + t14903 - t10398;
    (t14889, t14892, t14895, t14899, t14902, t14904)
}
