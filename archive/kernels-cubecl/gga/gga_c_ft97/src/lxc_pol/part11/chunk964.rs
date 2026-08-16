//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 964/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk964<F: Float>(t133: F, t1355: F, t139: F, t2001: F, t2043: F, t23866: F, t37696: F, t37699: F, t37702: F, t37704: F, t37707: F, t37709: F, t37712: F, t37715: F, t37718: F, t37720: F, t37763: F, t37768: F, t37772: F, t37774: F, t37776: F, t37778: F, t37781: F, t37785: F, t37787: F, t37792: F, t39835: F, t39839: F, t39843: F, t39847: F, t39849: F, t39852: F, t39854: F, t39917: F, t39985: F, t40012: F, t40036: F, t40046: F, t527: F, t538: F, t549: F, t550: F, t554: F, t8859: F, t8907: F, t8909: F, t8997: F) -> F {
    let t40051 = -F::cast_from(0.2416365355361531912e1_f64) * t1355 * t39835 + F::cast_from(0.44375043632495114232e3_f64) * t23866 * t39839 - F::cast_from(0.22187521816247557116e3_f64) * t8859 * t39843 - F::cast_from(0.14498192132169191472e2_f64) * t39847 * t39849 - F::cast_from(0.35032929183548774392e2_f64) * t39852 * t39854 - F::cast_from(8.0_f64) * t2001 * t549 * t8997 * t554 + F::cast_from(2.0_f64) * t527 * t139 * (t39917 + t39985) - t133 * t550 * (F::cast_from(0.60010200983333333334e0_f64) * t37696 - F::cast_from(0.13335600218518518519e0_f64) * t37699 - F::cast_from(0.80013601311111111114e0_f64) * t37702 + F::cast_from(0.10668480174814814815e1_f64) * t37704 - F::cast_from(0.1333560021851851852e0_f64) * t37707 - F::cast_from(0.71123201165432098768e0_f64) * t37709 + F::cast_from(0.88904001456790123462e-1_f64) * t37712 + F::cast_from(0.8890400145679012346e-1_f64) * t37715 + F::cast_from(0.31116400509876543211e0_f64) * t37718 + F::cast_from(0.97794401602469135807e0_f64) * t37720 + t40012 + F::cast_from(0.1333560021851851852e0_f64) * t37763 - F::cast_from(0.40006800655555555556e0_f64) * t37768 - F::cast_from(0.10001700163888888889e0_f64) * t37772 - F::cast_from(0.19558880320493827161e1_f64) * t37774 - F::cast_from(0.17780800291358024692e0_f64) * t37776 + F::cast_from(0.11853866860905349795e0_f64) * t37778 + F::cast_from(0.2469555596021947874e-1_f64) * t37781 - F::cast_from(0.30424924942990397807e1_f64) * t37785 + F::cast_from(0.65196267734979423872e0_f64) * t37787 + F::cast_from(0.80013601311111111114e0_f64) * t37792 + t40036) - F::cast_from(48.0_f64) * t2001 * t8907 * t538 * t8909 - F::cast_from(0.22955470875934553164e2_f64) * t2043 * t40046 + F::cast_from(0.22955470875934553164e2_f64) * t1355 * t40046;
    t40051
}
